//! Lost Days — `{4}{U}` instant — Lesson. "The owner of target creature or
//! enchantment puts it into their library second from the top or on the bottom.
//! You create a Clue token."
//!
//! # GAP: PutSecondFromTop — no Effect variant for placing a card second from
//!   the top (only PutOnTopOfLibrary / PutOnBottomOfLibrary exist)
//! # GAP: CreateClueToken — no pre-built Clue token helper; Clue has an
//!   activated ability ({2}, Sacrifice: Draw a card) not expressible in
//!   TokenDefinition.abilities

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lost Days");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "The owner of target creature or enchantment puts it into their library second from the top or on the bottom. You create a Clue token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine::CREATURE.into())
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: PutSecondFromTop — no Effect variant for placing a card second from top
    // GAP: CreateClueToken — Clue activated ability not expressible in TokenDefinition
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}
