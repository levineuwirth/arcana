//! Jet, Rebel Leader — `{3}{W}` 3/4 legendary white Human Rebel Ally.
//! "Whenever Jet attacks, look at the top five cards of your library. You
//! may put a creature card with mana value 3 or less from among them onto
//! the battlefield tapped and attacking. Put the rest on the bottom of your
//! library in a random order."
//!
//! GAP: no effect variant for "look at top N and put one onto battlefield
//! tapped and attacking". Using TutorToBattlefield as best approximation
//! (not exact: doesn't look at top N, doesn't make it attacking).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jet, Rebel Leader");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no effect for "look at top 5, put creature with cmc<=3 onto
    // battlefield tapped and attacking". TutorToBattlefield doesn't restrict
    // to top-N and doesn't make attacking.
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::creature().with_max_cmc(3),
        tapped: true,
    }]
}
