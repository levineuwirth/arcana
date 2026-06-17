//! Orcish Healer — `{R}{R}` 1/1 red Orc Cleric.
//! "{R}{R}, {T}: Target creature can't be regenerated this turn.
//!  {B}{B}{R}, {T}: Regenerate target black or green creature.
//!  {R}{G}{G}, {T}: Regenerate target black or green creature."
//!
//! The "can't be regenerated" ability has no expressible effect (GAP'd). The
//! two regenerate abilities are implemented; the "black or green" color
//! restriction on the target is approximated as target creature (no OR-color
//! target filter helper).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orcish Healer");
    let orc = reg.interner_mut().intern("Orc");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "{R}{R}, {T}: Target creature can't be regenerated this turn"
            // — no "can't be regenerated" effect available; ability omitted.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{B}{R}, {T}: Regenerate target black or green creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{B}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_target,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}{G}{G}, {T}: Regenerate target black or green creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{G}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_target,
            }),
    )
}

fn regenerate_target(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Regenerate { target: *id }]
}
