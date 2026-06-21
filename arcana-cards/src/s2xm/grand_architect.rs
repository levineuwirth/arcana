//! Grand Architect — `{1}{U}{U}` 1/3 Vedalken Artificer.
//!
//! Oracle:
//! * "Other blue creatures you control get +1/+1." — a continuous
//!   static anthem; not a triggered/activated ability, GAP'd.
//! * "{U}: Target artifact creature becomes blue until end of turn." —
//!   an activated color-change ability.
//! * "Tap an untapped blue creature you control: Add {C}{C}. Spend
//!   this mana only to cast artifact spells or activate abilities of
//!   artifacts." — a mana ability whose cost taps another blue
//!   creature. The "spend only on artifacts" restriction has no
//!   primitive and is a documented fidelity gap.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grand Architect");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "Other blue creatures you control get +1/+1" — a
    // continuous anthem, not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}: Target artifact creature becomes blue until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_blue,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap an untapped blue creature you control: Add {C}{C}.".into(),
                cost: ActivationCost {
                    tap_other: Some(
                        ObjectFilter::creature()
                            .with_colors(ColorSet::blue())
                            .controlled_by(ControllerConstraint::You)
                            .untapped_only(),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                // GAP fidelity: "Spend this mana only to cast artifact
                // spells or activate abilities of artifacts" — no
                // restricted-mana primitive; plain colorless is added.
                effect: add_two_colorless,
            }),
    )
}

fn make_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::SetColor {
        target: *id,
        colors: ColorSet::blue(),
        duration: Duration::EndOfTurn,
    }]
}

fn add_two_colorless(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); 2],
    }]
}
