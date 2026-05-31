//! Nessian Game Warden — `{3}{G}{G}` 4/5 green Beast. "When this
//! creature enters, look at the top X cards of your library, where X
//! is the number of Forests you control. You may reveal a creature
//! card from among them and put it into your hand. Put the rest on the
//! bottom of your library in any order."

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nessian Game Warden");
    let beast = reg.interner_mut().intern("Beast");
    // Interned at register so the resolver's Forest filter resolves.
    let _forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_for_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_dig_for_creature(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // X is the number of Forests you control.
    let forests = script::subtype_filter(reg, "Forest");
    let x = script::count_matching(state, &forests, trig.controller);
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::DigTopN {
        player: trig.controller,
        count: x,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}
