//! Mog, Moogle Warrior — `{1}{R}{W}` 1/2 Legendary Moogle Warrior.
//! Lifelink.
//!
//! Dance — At the beginning of your end step, each player may discard a
//! card. Each player who discarded draws a card. If a creature card was
//! discarded, create a 1/2 white Moogle token with lifelink; then if a
//! noncreature card was discarded, put a +1/+1 counter on each Moogle
//! you control.
//!
//! The "each player MAY discard, then branch on what types were
//! discarded" structure has no demonstrated primitive (no per-player
//! optional discard with discarded-type tracking), so the effect body
//! is GAP'd; the lifelink keyword and the end-step trigger hook stand.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mog, Moogle Warrior");
    let moogle = reg.interner_mut().intern("Moogle");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moogle);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: dance,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn dance(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each player may discard a card; each who did draws; if a creature card was discarded
    // create a Moogle; then if a noncreature was discarded buff each Moogle" — no per-player
    // optional-discard with discarded-type tracking primitive.
    Vec::new()
}
