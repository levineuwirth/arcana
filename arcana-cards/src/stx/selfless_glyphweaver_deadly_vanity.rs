//! Selfless Glyphweaver // Deadly Vanity
//!
//! Front: Creature — Human Cleric {2}{W} 2/3 (white)
//!   Exile this creature: Creatures you control gain indestructible until end of turn.
//! Back: Sorcery
//!   Choose a creature or planeswalker, then destroy all other creatures and planeswalkers.
//! GAP: "Exile this creature as cost: effect" not modeled (activated ability with exile-self cost)
//! GAP: Back face "destroy all other creatures and planeswalkers" not in Effect catalog as named target exception

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Selfless Glyphweaver");
    let back_name = reg.interner_mut().intern("Deadly Vanity");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(human);
    subtypes.insert(cleric);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{7}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };

    // Back: choose a creature or planeswalker, destroy all others
    let back_ability = SpellAbilityDef {
        text: "Choose a creature or planeswalker, then destroy all other creatures and planeswalkers.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Creature,
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: back_resolve,
    };

    reg.register(
        CardDefinition::new(name, chars).with_mdfc_back(CardFace {
            name: back_name,
            characteristics: back_chars,
            spell_ability: Some(back_ability),
        }),
    )
}

fn back_resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(chosen_id) = target else {
        return Vec::new();
    };
    let chosen = *chosen_id;

    // Destroy all creatures except the chosen one
    let filter = ObjectFilter::creature();
    let ids = arcana_core::script::ids_matching(state, &filter, entry.controller);
    ids.into_iter()
        .filter(|id| *id != chosen)
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect()
}
