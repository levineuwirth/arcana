//! Lurker in the Deep — `{3}{U}{U}{U}` 7/7 Fish Illusion Enchantment Creature.
//! Impending 3—{2}{U}{U}.
//! "Whenever Lurker in the Deep enters or attacks, seek a nonland card."
//! "Whenever you seek one or more cards during your turn, conjure a duplicate of
//! each of those cards into your hand, then manifest those duplicates."
//!
//! GAP: Impending / Seek / Manifest / Conjure are not usable `KeywordAbility`
//! variants — keywords omitted; the card is the standard creature form.
//! GAP: "seek a nonland card" has no `Effect` variant (Seek is unmodeled) — the
//! enters/attacks trigger fires but its effect body is empty.
//! GAP: "Whenever you seek one or more cards…" has no matching `TriggerCondition`
//! and Conjure is not modeled — the second trigger is omitted entirely.

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
    let name = reg.interner_mut().intern("Lurker in the Deep");
    let fish = reg.interner_mut().intern("Fish");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: seek_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: seek_nonland,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn seek_nonland(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek a nonland card" — no `Effect::Seek` variant exists; omitted.
    Vec::new()
}
