//! Banquet Guests — `{X}{G}{W}` 0/0 Halfling Citizen with Trample.
//! Affinity for Foods; "enters with twice X +1/+1 counters"; and
//! "{2}, Sacrifice a Food: This creature gains indestructible until end
//! of turn."
//!
//! Affinity is not a usable keyword (cost reduction is unmodeled) and
//! the enters-with-twice-X counters has no expressible primitive — both
//! GAP'd. Trample and the sac-a-Food activation are wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: Affinity for Foods (cost reduction) — Affinity is not an available
//      keyword; emit no keyword for it.
// GAP: "This creature enters with twice X +1/+1 counters on it." — no
//      enters-with-counters primitive (and X-doubling); omit.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Banquet Guests");
    let halfling = reg.interner_mut().intern("Halfling");
    let citizen = reg.interner_mut().intern("Citizen");
    let food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(citizen);

    let food_filter = ObjectFilter {
        subtypes: Some(vec![food]),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Sacrifice a Food: This creature gains indestructible until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    sacrifice_other: Some(food_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_indestructible,
            }),
    )
}

fn gain_indestructible(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Indestructible,
        duration: Duration::EndOfTurn,
    }]
}
