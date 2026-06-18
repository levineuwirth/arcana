//! Vrestin, Menoptra Leader — `{X}{G}{G}{W}` 0/0 Legendary Alien Insect Scout
//! with Flying.
//!
//! Oracle:
//! * Flying — base keyword.
//! * "Vrestin enters with X +1/+1 counters on it." The count X is the value
//!   paid for {X} at cast time; no demonstrated trigger accessor / dynamic-X
//!   closure exposes the cast X to an ETB resolver, so the counter count is
//!   not computable. GAP.
//! * "When Vrestin enters, create X 1/1 green and white Alien Insect creature
//!   tokens with flying." Same cast-X dependency — the token count cannot be
//!   computed from the demonstrated accessors. GAP (the ETB trigger shell is
//!   emitted with an empty resolver so the hook is recorded).
//! * "Whenever you attack with one or more Insects, put a +1/+1 counter on
//!   each of them." Modeled as a per-attacking-Insect `CreatureAttacks`
//!   trigger that puts a +1/+1 counter on the attacking Insect.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vrestin, Menoptra Leader");
    let alien = reg.interner_mut().intern("Alien");
    let insect = reg.interner_mut().intern("Insect");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(insect);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: ETB rider — "Vrestin enters with X +1/+1 counters on it." X is the
    // cast {X} value, not exposed to an ETB resolver by the demonstrated
    // accessors.
    let insect_filter =
        script::subtype_filter(reg, "Insect").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_x_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: insect_filter,
                },
                intervening_if: None,
                effect: on_insect_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_x_tokens(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "create X 1/1 green and white Alien Insect tokens with flying." The
    // token count X is the cast {X} value; no demonstrated accessor exposes it
    // to the ETB resolver, so the count cannot be computed and the effect is
    // withheld rather than minting a wrong fixed number.
    Vec::new()
}

fn on_insect_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "put a +1/+1 counter on each of them" — the CreatureAttacks trigger
    // fires per attacking Insect; place the counter on the attacking Insect.
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
