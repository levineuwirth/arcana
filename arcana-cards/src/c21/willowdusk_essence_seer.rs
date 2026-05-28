//! Willowdusk, Essence Seer — `{1}{B}{G}` 3/3 Legendary Creature — Dryad Druid.
//! `{1}, {T}: Put +1/+1 counters on target creature equal to life gained or lost this turn (whichever is greater). Sorcery speed.`
//! GAP: no script helper for "life gained this turn" or "life lost this turn".

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Willowdusk, Essence Seer");
    let dryad = reg.interner_mut().intern("Dryad");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Put +1/+1 counters on target creature equal to max(life gained, life lost) this turn.".into(),
                cost: ActivationCost { mana_cost: ManaCost::parse("{1}").unwrap(), tap: true, ..ActivationCost::default() },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_counters,
            }),
    )
}

fn put_counters(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no script helper for "life gained this turn" or "life lost this turn"
    Vec::new()
}
