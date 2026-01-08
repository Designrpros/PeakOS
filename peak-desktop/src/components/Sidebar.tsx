
import { TreeItem, getDynamicTree, TreeItemData } from './SidebarTree';
import { useOSStore } from '../stores/useOSStore';

interface SidebarProps {
    activeId?: string;
    onSelect?: (item: TreeItemData) => void;
}

export function Sidebar({ activeId = 'desktop', onSelect }: SidebarProps) {
    const { shortcuts } = useOSStore();
    const treeData = getDynamicTree(shortcuts);

    return (
        <div className="w-full h-full hidden md:flex flex-col bg-background/40 backdrop-blur-xl border-r border-border/50 text-foreground transition-all duration-300 pt-10">
            <div className="p-2 space-y-1 overflow-y-auto">
                {treeData.map((item) => (
                    <TreeItem
                        key={item.id}
                        item={item}
                        activeId={activeId}
                        onSelect={onSelect}
                    />
                ))}
            </div>
        </div>
    );
}
