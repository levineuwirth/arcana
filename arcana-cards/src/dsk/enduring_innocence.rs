//! Enduring Innocence — `{1}{W}{W}` 2/1 Enchantment Creature — Sheep Glimmer.
//! Lifelink.
//! Whenever one or more other creatures you control with power 2 or less enter,
//! draw a card. This ability triggers only once each turn.
//! When Enduring Innocence dies, if it was a creature, return it to the
//! battlefield under its owner's control. It's an enchantment (not a creature).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enduring Innocence");
    let sheep = reg.interner_mut().intern("Sheep");
    let glimmer = reg.interner_mut().intern("Glimmer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sheep);
    subtypes.0.insert(glimmer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_max_power(2),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: draw_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: return_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

fn return_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it was a creature" (intervening-if on the dying face) and
    // "It's an enchantment (not a creature)" — returning the card stripped of
    // its creature type is not expressible; we return it as printed.
    let id = trig.dying_object().unwrap_or(trig.source);
    vec![Effect::ReturnFromGraveyardToBattlefield { target: id }]
}
