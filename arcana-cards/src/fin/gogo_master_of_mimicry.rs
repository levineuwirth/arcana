//! Gogo, Master of Mimicry — `{2}{U}` 2/4 legendary blue Wizard.
//! "{X}{X}, {T}: Copy target activated or triggered ability you control X
//! times. You may choose new targets for the copies."
//! Targeting an activated/triggered ability is wired via
//! TargetFilter::AbilityOnStack; GAP: copying the ability X times is not in
//! the Effect catalog (Effect::Counter is the only ability-entry consumer).
//! The {X}{X} cost is expressible, but with the copy-ability effect missing
//! the ability stays GAP'd (cost left unwired); emitting Vec::new().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gogo, Master of Mimicry");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{X}, {T}: Copy target activated or triggered ability you control X times. You may choose new targets for the copies.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: "you control" — ability stack entries are not
                // GameObjects, so the outer controller constraint can't
                // see their controller; left unconstrained.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AbilityOnStack {
                        activated: true,
                        triggered: true,
                        source_filter: None,
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_ability,
            }),
    )
}

fn copy_ability(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: copying an activated or triggered ability X times is not in the
    // Effect catalog (CopySpell only copies spells, not abilities; the
    // AbilityOnStack target is only consumable by Effect::Counter). The
    // {X}{X} cost is now expressible, but with no copy-ability primitive the
    // whole effect stays GAP'd, so the cost is left unwired rather than
    // charging mana for a no-op.
    Vec::new()
}
