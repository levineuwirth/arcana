//! Will, Scion of Peace — `{1}{W}{U}` 2/4 Legendary Human Wizard.
//! Vigilance.
//! {T}: Spells you cast this turn that are white and/or blue cost {X}
//! less to cast, where X is the amount of life you gained this turn.
//! Activate only as a sorcery.
//!
//! Vigilance is wired. The {T} activated ability is wired (tap cost,
//! sorcery-speed), but its effect — a dynamic, color-filtered spell
//! cost-reduction for the rest of the turn — has no available primitive,
//! so the effect body is GAP'd (empty).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Will, Scion of Peace");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Spells you cast this turn that are white and/or blue cost \
                   {X} less to cast, where X is the amount of life you gained this \
                   turn. Activate only as a sorcery."
                .into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: cost_reduction,
        }),
    )
}

fn cost_reduction(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic, color-filtered, rest-of-turn spell cost-reduction
    // (X = life gained this turn) — no cost-reduction primitive available.
    Vec::new()
}
