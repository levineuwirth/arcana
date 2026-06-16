//! Sunderflock — `{7}{U}{U}` 5/5 blue Elemental with Flying.
//!
//! Oracle text:
//! * "This spell costs {X} less to cast, where X is the greatest mana
//!   value among Elementals you control." — GAP: no cost-reduction
//!   primitive in the demonstrated API.
//! * Flying.
//! * "When this creature enters, if you cast it, return all
//!   non-Elemental creatures to their owners' hands." — the
//!   "if you cast it" cast-condition gate is NOT expressible, so the
//!   trigger fires on every ETB; the bounce of all non-Elemental
//!   creatures IS expressible via ForEach over the matching ids.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunderflock");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "This spell costs {X} less to cast" — no cost-reduction primitive.
    reg.register(
        CardDefinition::new(name, chars)
            // GAP intervening-if "if you cast it" — no cast-condition gate;
            // trigger fires on every ETB instead.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_bounce_non_elementals,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_bounce_non_elementals(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(elemental) = reg.interner().lookup("Elemental") else {
        return Vec::new();
    };
    let filter = ObjectFilter::creature().without_subtype_sym(elemental);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnToHand {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
