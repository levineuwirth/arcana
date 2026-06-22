//! Piper of the Swarm — `{1}{B}` 1/3 Human Warlock.
//! "Rats you control have menace.
//!  {1}{B}, {T}: Create a 1/1 black Rat creature token.
//!  {2}{B}{B}, {T}, Sacrifice three Rats: Gain control of target creature."
//!
//! "Rats you control have menace" is a static keyword-grant — GAP'd (no
//! expressible static-grant primitive). The two activated abilities are wired:
//! a mana+tap token-maker, and a mana+tap+sacrifice-three-Rats permanent
//! gain-control of a target creature (sacrifice_other with count 3).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Piper of the Swarm");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let _rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "Rats you control have menace." — static keyword-grant, not a
    // triggered/activated ability.

    let rat_filter = script::subtype_filter(reg, "Rat");

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}, {T}: Create a 1/1 black Rat creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_rat,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}{B}, {T}, Sacrifice three Rats: Gain control of \
                       target creature."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}{B}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(rat_filter),
                    sacrifice_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_control,
            }),
    )
}

fn make_rat(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rat = match reg.interner().lookup("Rat") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: rat,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn gain_control(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: ctx.controller,
    }]
}
