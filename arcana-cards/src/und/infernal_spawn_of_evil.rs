//! Infernal Spawn of Evil — `{6}{B}{B}{B}` 7/7 Beast with Flying and First strike.
//! `{1}{B}, Reveal this card from your hand, Say "It's coming": This card deals
//!  1 damage to target opponent or planeswalker. Activate only during your
//!  upkeep and only once each turn.`
//!
//! The activated ability is wired from the hand with `once_per_turn`. The
//! "reveal this card" / "say It's coming" flavor costs and the "only during your
//! upkeep" timing restriction are not expressible cost/timing fields (GAP). The
//! "opponent or planeswalker" target restriction is approximated with
//! `any_target()` (GAP).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Infernal Spawn of Evil");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{B}, Reveal this card from your hand, Say \"It's coming\": This card deals 1 damage to target opponent or planeswalker. Activate only during your upkeep and only once each turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                once_per_turn: true,
                // GAP: "Reveal this card from your hand" / "Say It's coming" are
                // not expressible cost fields; "only during your upkeep" timing
                // restriction has no cost/timing field.
                ..ActivationCost::default()
            },
            // GAP: target should be "opponent or planeswalker"; approximated with
            // any_target.
            target_requirements: vec![TargetRequirement::any_target()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Hand,
            is_instant_speed: false,
            face_gate: None,
            effect: deal_one,
        }),
    )
}

fn deal_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 1,
    }]
}
