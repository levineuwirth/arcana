//! Reverent Hoplite — `{4}{W}` 1/2 white Human Soldier.
//! "When this creature enters, create a number of 1/1 white Human Soldier creature tokens
//! equal to your devotion to white."
//! Uses script::devotion for dynamic token count.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reverent Hoplite");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    // Pre-intern token subtypes for resolver lookup
    let _human_tok = reg.interner_mut().intern("Human");
    let _soldier_tok = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                effect: etb_create_soldier_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_soldier_tokens(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::devotion(state, trig.controller, ColorSet::white());
    if n == 0 {
        return Vec::new();
    }
    let human = reg.interner().lookup("Human").expect("Human interned during register()");
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned during register()");
    let mut effects = Vec::new();
    for _ in 0..n {
        let mut st = SubtypeSet::default();
        st.0.insert(human);
        st.0.insert(soldier);
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: soldier,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: st,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}
