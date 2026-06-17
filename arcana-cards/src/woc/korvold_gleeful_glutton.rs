//! Korvold, Gleeful Glutton — `{5}{B}{R}{G}` 4/4 Legendary Dragon Noble
//! with Flying, Trample, Haste. Its cost-reduction static is GAP'd. When
//! it deals combat damage to a player, put X +1/+1 counters on it and draw
//! X cards, where X = the number of permanent card types among cards in
//! your graveyard — that distinct-type count is not computable with the
//! documented script:: helpers, so the trigger effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Korvold, Gleeful Glutton");
    let dragon = reg.interner_mut().intern("Dragon");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(noble);

    // GAP: "This spell costs {1} less to cast for each card type among
    // permanents you've sacrificed this turn" — dynamic cost reduction not
    // expressible via the documented card-class surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Trample,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: korvold_combat_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn korvold_combat_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X = "the number of permanent types among cards in your
    // graveyard" — distinct-card-type counting over a graveyard is not
    // computable with the documented script:: helpers; the whole effect is
    // GAP'd rather than emit a wrong literal amount.
    Vec::new()
}
