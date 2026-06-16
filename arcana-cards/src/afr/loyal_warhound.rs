//! Loyal Warhound — `{1}{W}` 3/1 Dog with Vigilance.
//! "When this creature enters, if an opponent controls more lands than
//! you, search your library for a basic Plains card, put it onto the
//! battlefield tapped, then shuffle."
//!
//! The ETB land-fetch is gated by an intervening-if comparing land
//! counts; the comparison ("an opponent controls MORE lands than you")
//! is not expressible with the available `conditions::` predicates, so
//! the gate is GAP'd and the search fires unconditionally (best-effort).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loyal Warhound");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if an opponent controls more lands than you"
            // — no conditions:: predicate compares your land count to an
            // opponent's; gate omitted, search fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_fetch_plains,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_fetch_plains(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut filter = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    if let Some(sub) = reg.interner().lookup("Plains") {
        filter = filter.with_subtype_sym(sub);
    }
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter,
        tapped: true,
    }]
}
