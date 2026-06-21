//! Knight of the White Orchid — `{W}{W}` 2/2 Human Knight with First
//! strike.
//! "When this creature enters, if an opponent controls more lands than
//!  you, you may search your library for a Plains card, put it onto the
//!  battlefield, then shuffle."
//!
//! First strike is a base keyword. The ETB is gated by an intervening-if
//! comparing your land count to each opponent's; the effect tutors a
//! Plains card onto the battlefield (untapped).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of the White Orchid");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: Some(if_opponent_controls_more_lands),
            effect: etb_tutor_plains,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_opponent_controls_more_lands(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    let land = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    let your_lands = script::count_matching(s, &land, you);
    script::opponents(s, you)
        .into_iter()
        .any(|opp| script::count_matching(s, &land, opp) > your_lands)
}

fn etb_tutor_plains(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plains = script::subtype_filter(reg, "Plains");
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: plains,
        tapped: false,
    }]
}
