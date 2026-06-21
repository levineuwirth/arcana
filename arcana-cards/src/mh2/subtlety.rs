//! Subtlety — `{2}{U}{U}` 3/3 Elemental Incarnation with Flash and Flying.
//!
//! "When this creature enters, choose up to one target creature spell or
//! planeswalker spell. Its owner puts it on their choice of the top or
//! bottom of their library."
//! "Evoke—Exile a blue card from your hand." (Evoke is not a documented
//! keyword/mechanic — GAP'd)
//!
//! The ETB targets up to one creature/planeswalker spell on the stack and
//! puts it on top of its owner's library. The owner's "top or bottom" choice
//! is GAP'd (no documented way to express it) — top is chosen.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP (keyword/mechanic): "Evoke—Exile a blue card from your hand." — Evoke is
// outside the documented usable keyword/alternative-cost surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Subtlety");
    let elemental = reg.interner_mut().intern("Elemental");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(incarnation);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: send_spell_to_library,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn send_spell_to_library(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP (choice): "their choice of the top or bottom of their library" — the
    // owner's top/bottom choice is not expressible; top is used.
    vec![Effect::PutOnTopOfLibrary { target: *id }]
}
