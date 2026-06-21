//! Outcaster Greenblade — `{2}{G}` 1/2 Human Mercenary.
//!
//! Oracle:
//! * When this creature enters, search your library for a basic land card or
//!   a Desert card, reveal it, put it into your hand, then shuffle.
//! * This creature gets +1/+1 for each Desert you control.
//!
//! The ETB tutor is modeled as a search for a basic land card (supertype
//! Basic + type Land); the "or a Desert card" disjunction can't be expressed
//! in one filter (basic-supertype OR Desert-subtype), so that alternative is
//! GAP-noted. The static +1/+1-per-Desert P/T modifier has no primitive and
//! is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Outcaster Greenblade");
    let human = reg.interner_mut().intern("Human");
    let merc = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(merc);

    // GAP: static — "gets +1/+1 for each Desert you control" (dynamic P/T modifier).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_tutor_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP partial: "or a Desert card" — the basic-land OR Desert disjunction
    // can't be a single filter; modeled here as the basic-land branch.
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    vec![Effect::TutorToHand { player: trig.controller, filter, reveal: true }]
}
