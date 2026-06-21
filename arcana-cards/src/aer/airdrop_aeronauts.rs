//! Airdrop Aeronauts — `{3}{W}{W}` 4/3 Creature — Dwarf Scout. White.
//! Flying.
//! "Revolt — When this creature enters, if a permanent left the
//!  battlefield under your control this turn, you gain 5 life."
//!
//! The ETB trigger is wired, but its effect is GAP'd: the Revolt gate
//! ("if a permanent left the battlefield under your control this turn")
//! has no `conditions::` predicate, so it can be neither set as an
//! intervening-if nor checked at resolution. Firing the life gain
//! unconditionally would materially overstate the card, so the gated
//! effect is omitted. (Revolt is an ability word, not a keyword line.)

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Airdrop Aeronauts");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: revolt_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn revolt_gain_life(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Revolt gate ("a permanent left the battlefield under your
    // control this turn") has no conditions:: predicate; the gain-5-life
    // is conditional and would be wrong if fired unconditionally.
    Vec::new()
}
