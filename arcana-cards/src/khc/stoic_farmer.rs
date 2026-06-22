//! Stoic Farmer — `{3}{W}` 3/3 Dwarf Peasant. Foretell.
//! "When this creature enters, search your library for a basic Plains card
//! and reveal it. If an opponent controls more lands than you, put it onto
//! the battlefield tapped. Otherwise, put it into your hand. Then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stoic Farmer");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let peasant = reg.interner_mut().intern("Peasant");
    // Interned at register so the tutor resolver can build the Plains filter.
    let _plains = reg.interner_mut().intern("Plains");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(peasant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Foretell — not in the usable KeywordAbility surface; the
        // face-down-exile cast mechanic is omitted.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
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
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(plains) = reg.interner().lookup("Plains") else { return Vec::new(); };
    let basic_plains = ObjectFilter {
        types: Some(TypeLine::LAND.into()),
        supertypes: Some(SupertypeSet(SupertypeSet::BASIC)),
        subtypes: Some(vec![plains]),
        ..ObjectFilter::default()
    };

    let land_filter = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    let my_lands = script::count_matching(
        state,
        &land_filter.clone().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let opp_lands = script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| {
            script::count_matching(
                state,
                &land_filter.clone().controlled_by(ControllerConstraint::You),
                opp,
            )
        })
        .max()
        .unwrap_or(0);

    if opp_lands > my_lands {
        vec![Effect::TutorToBattlefield {
            player: trig.controller,
            filter: basic_plains,
            tapped: true,
        }]
    } else {
        vec![Effect::TutorToHand {
            player: trig.controller,
            filter: basic_plains,
            reveal: true,
        }]
    }
}
