//! Zhentarim Bandit — `{1}{B}` 2/1 black creature. "Whenever this creature
//! attacks, you may pay 1 life. If you do, create a Treasure token."
//!
//! GAP: effect — optional "pay 1 life" gate before Treasure creation. Emitting
//! LoseLife + CreateToken unconditionally.

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
    let name = reg.interner_mut().intern("Zhentarim Bandit");
    let halfling = reg.interner_mut().intern("Halfling");
    let rogue = reg.interner_mut().intern("Rogue");
    let _treasure = reg.interner_mut().intern("Treasure");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: effect — optional "pay 1 life" gate; emitting unconditionally
    let treasure = reg.interner().lookup("Treasure")
        .expect("Treasure interned during register()");
    let mut st = SubtypeSet::default();
    st.0.insert(treasure);
    let token = TokenDefinition {
        name: treasure,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: st,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::LoseLife { player: trig.controller, amount: 1 },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
