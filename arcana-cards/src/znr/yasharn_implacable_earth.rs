//! Yasharn, Implacable Earth — `{2}{G}{W}` 4/4 Legendary Elemental Boar.
//!
//! * "When Yasharn enters, search your library for a basic Forest card
//!   and a basic Plains card, reveal those cards, put them into your
//!   hand, then shuffle." (ETB; two TutorToHand effects in a Sequence)
//! * "Players can't pay life or sacrifice nonland permanents to cast
//!   spells or activate abilities." — a pure static rule-altering
//!   ability with no trigger or cost; not expressible. GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yasharn, Implacable Earth");
    let elemental = reg.interner_mut().intern("Elemental");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(boar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: "Players can't pay life or sacrifice nonland permanents to
    // cast spells or activate abilities." — pure rule-altering static.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_fetch_lands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_fetch_lands(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let basic = SupertypeSet::new().with(SupertypeSet::BASIC);
    let forest = script::subtype_filter(reg, "Forest").with_supertypes(basic);
    let plains = script::subtype_filter(reg, "Plains").with_supertypes(basic);
    vec![Effect::Sequence(vec![
        Effect::TutorToHand {
            player: trig.controller,
            filter: forest,
            reveal: true,
        },
        Effect::TutorToHand {
            player: trig.controller,
            filter: plains,
            reveal: true,
        },
    ])]
}
