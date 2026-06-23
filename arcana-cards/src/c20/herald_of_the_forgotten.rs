//! Herald of the Forgotten — `{6}{W}{W}` 6/6 Cat Beast with Flying.
//!
//! Oracle:
//! * Flying
//! * When this creature enters, if you cast it, return any number of
//!   target permanent cards with cycling abilities from your graveyard
//!   to the battlefield.
//!
//! Implemented: Flying keyword + the ETB reanimation trigger. The trigger
//! takes "any number of" target cards in the controller's graveyard
//! (`TargetCount::Any`) and returns each chosen card to the battlefield
//! (`ReturnFromGraveyardToBattlefield`, one per chosen target, wrapped in
//! a `Sequence`).
//!
//! GAPs (target-filter refinements that aren't expressible, so the
//! requirement targets any graveyard card):
//! * "if you cast it" — cast-vs-otherwise-entering intervening-if is not
//!   an available condition predicate.
//! * "permanent cards with cycling abilities" — `ObjectFilter` cannot
//!   express "has a cycling ability", so the cycling restriction (and the
//!   permanent-card narrowing) is dropped.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Herald of the Forgotten");
    let cat = reg.interner_mut().intern("Cat");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: intervening-if — "if you cast it" (cast-vs-entered gate) is not
    // an available condition predicate; the trigger fires on any entry.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_reanimate_targets,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    // GAP: cannot filter "permanent card with a cycling
                    // ability" — any graveyard card is targetable.
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default(),
                    },
                    count: TargetCount::Any,
                    controller: None,
                }],
            }),
    )
}

fn etb_reanimate_targets(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let effects: Vec<Effect> = trig
        .targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => {
                Some(Effect::ReturnFromGraveyardToBattlefield { target: *id })
            }
            _ => None,
        })
        .collect();
    if effects.is_empty() {
        return Vec::new();
    }
    vec![Effect::Sequence(effects)]
}
