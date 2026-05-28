//! Omniclown Colossus // Pie-roclasm — `{8}{R}{R}` / `{3}{R}{R}` Adventure
//!
//! Creature: `{8}{R}{R}` Artifact Creature — Clown Robot (7/7)
//!   Affinity for Clowns (cost reduction, GAP).
//!   Trample.
//!
//! Adventure: `{3}{R}{R}` Sorcery — Pie-roclasm
//!   Deals π (≈3) damage to each non-Clown creature.
//!   (GAP: "non-Clown" filter not expressible; emitting 3 damage to all.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omniclown Colossus");
    let clown_sub = reg.interner_mut().intern("Clown");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(clown_sub);
    subtypes.0.insert(robot_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        keywords: vec![KeywordAbility::Trample],
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Pie-roclasm");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "This creature deals pi damage to each non-Clown creature.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature();
    let ids = script::ids_matching(state, &filter, entry.controller);
    ids.into_iter()
        .map(|id| Effect::DealDamage { target: DamageTarget::Object(id), amount: 3, source: entry.source })
        .collect()
}
