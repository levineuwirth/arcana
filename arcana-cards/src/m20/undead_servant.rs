//! Undead Servant — `{3}{B}` 3/2 black creature. "When this creature enters,
//! create a 2/2 black Zombie creature token for each card named Undead Servant
//! in your graveyard."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undead Servant");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_zombie_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_zombie_tokens(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no named-card-in-graveyard count helper; using graveyard_size as approximation is wrong
    // Use script::graveyard_size as a fallback count (incorrect — should count "Undead Servant" cards)
    // GAP: script has no way to filter graveyard by card name
    let zombie = reg.interner().lookup("Zombie").expect("Zombie interned during register()");
    let n = arcana_core::script::graveyard_size(state, trig.controller);
    let mut effects = Vec::new();
    for _ in 0..n {
        let mut st = SubtypeSet::default();
        st.0.insert(zombie);
        let token = TokenDefinition {
            name: zombie,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: st,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        };
        effects.push(Effect::CreateToken { controller: trig.controller, token });
    }
    effects
}
