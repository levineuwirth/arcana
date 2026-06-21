//! Catharsis — `{4}{R/W}{R/W}` 3/4 red/white Elemental Incarnation.
//!
//! * ETB: if {W}{W} was spent to cast it, create two 1/1 G/W Kithkin.
//! * ETB: if {R}{R} was spent to cast it, your creatures get +1/+1 and
//!   gain haste until end of turn.
//! * Evoke {R/W}{R/W}. (No KeywordAbility::Evoke variant — GAP.)
//!
//! Both ETBs are gated on "if {W}{W}/{R}{R} was spent to cast it", an
//! intervening-if with no `conditions::` predicate (no mana-spent state
//! accessor). Firing the payloads unconditionally would be materially
//! wrong, so the gated effects are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Catharsis");
    let elemental = reg.interner_mut().intern("Elemental");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(incarnation);

    // GAP: Evoke {R/W}{R/W} — no KeywordAbility::Evoke variant.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R/W}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_white,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_red,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_white(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: gated on "if {W}{W} was spent to cast it" — no mana-spent condition
    // predicate, so the token-creation payload cannot be conditioned.
    Vec::new()
}

fn etb_red(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: gated on "if {R}{R} was spent to cast it" — no mana-spent condition
    // predicate, so the anthem/haste payload cannot be conditioned.
    Vec::new()
}
