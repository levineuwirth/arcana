//! Yuna, Grand Summoner — `{1}{G}{W}{U}` 1/5 Legendary green-white-blue
//! Human Cleric.
//!
//! Oracle:
//! * "Grand Summon — {T}: Add one mana of any color. When you next cast a
//!   creature spell this turn, that creature enters with two additional
//!   +1/+1 counters on it." — modeled as five {T} abilities, one per WUBRG
//!   color (the player picks the color by choosing which ability to activate;
//!   the shared {T} cost means only one fires). The "when you next cast a
//!   creature spell" rider is GAP'd: there is no next-cast-rider primitive in
//!   the demonstrated effect surface (and the engine's single rider only adds
//!   ONE counter, not "two additional"). Because of the non-mana rider clause
//!   these are not flagged as pure mana abilities.
//! * "Whenever another permanent you control is put into a graveyard from
//!   the battlefield, if it had one or more counters on it, you may put
//!   that number of +1/+1 counters on target creature." — a battlefield ->
//!   graveyard ZoneChange trigger on another permanent you control,
//!   targeting a creature. GAP (effect body): the dynamic count ("that
//!   number" = the counters that were on the dying permanent) has no script
//!   helper to read counters on a post-death object, and the "if it had one
//!   or more counters" gate is likewise uncheckable post-death; emitting a
//!   literal counter count would be unfaithful, so the effect is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yuna, Grand Summoner");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(grand_summon_ability(
                "Grand Summon — {T}: Add {W}. When you next cast a creature spell this turn, that creature enters with two additional +1/+1 counters on it.",
                grand_summon_white,
            ))
            .with_activated_ability(grand_summon_ability(
                "Grand Summon — {T}: Add {U}. When you next cast a creature spell this turn, that creature enters with two additional +1/+1 counters on it.",
                grand_summon_blue,
            ))
            .with_activated_ability(grand_summon_ability(
                "Grand Summon — {T}: Add {B}. When you next cast a creature spell this turn, that creature enters with two additional +1/+1 counters on it.",
                grand_summon_black,
            ))
            .with_activated_ability(grand_summon_ability(
                "Grand Summon — {T}: Add {R}. When you next cast a creature spell this turn, that creature enters with two additional +1/+1 counters on it.",
                grand_summon_red,
            ))
            .with_activated_ability(grand_summon_ability(
                "Grand Summon — {T}: Add {G}. When you next cast a creature spell this turn, that creature enters with two additional +1/+1 counters on it.",
                grand_summon_green,
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                // GAP (intervening-if): "if it had one or more counters on it"
                // cannot be checked post-death; gate omitted.
                intervening_if: None,
                effect: counters_on_death,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn grand_summon_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    // GAP: "when you next cast a creature spell this turn, that creature
    // enters with two additional +1/+1 counters" — no next-cast-rider
    // primitive in the demonstrated surface (and no two-counter rider). Only
    // the mana production is wired; the non-mana rider keeps this off the
    // pure-mana-ability flag.
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn grand_summon_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)] }]
}
fn grand_summon_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)] }]
}
fn grand_summon_black(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)] }]
}
fn grand_summon_red(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)] }]
}
fn grand_summon_green(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)] }]
}

fn counters_on_death(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put that number of +1/+1 counters on target creature" where the
    // number is the counters that were on the dying permanent — no script
    // helper reads counters on a post-death object, so the dynamic count is
    // not computable; the whole effect is omitted rather than emit a literal.
    Vec::new()
}
