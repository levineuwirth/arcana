//! Fleetfeather Cockatrice — `{3}{G}{U}` 3/3 Cockatrice with Flash, Flying,
//! Deathtouch. "{5}{G}{U}: Monstrosity 3." There is no Monstrosity effect, so
//! the becomes-monstrous flag is GAP'd; the mechanical payload (put three
//! +1/+1 counters on it) is emitted as the activated ability.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fleetfeather Cockatrice");
    let cockatrice = reg.interner_mut().intern("Cockatrice");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cockatrice);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::Flying,
            KeywordAbility::Deathtouch,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            // GAP: the "becomes monstrous" flag (and the once-only gate) is not
            // modeled; only the +1/+1 counter payload is emitted.
            text: "{5}{G}{U}: Monstrosity 3.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}{G}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: monstrosity_3,
        }),
    )
}

fn monstrosity_3(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }]
}
