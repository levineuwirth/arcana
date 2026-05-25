//! Blight Herder — `{5}` 4/5 colorless Eldrazi Processor.
//! "When you cast this spell, you may put two cards your opponents own from
//! exile into their owners' graveyards. If you do, create three 1/1
//! colorless Eldrazi Scion creature tokens. They have 'Sacrifice this token:
//! Add {C}.'"
//! GAP: trigger — "when you cast this spell" fires on cast, before it
//! resolves; no TriggerCondition for self-cast. Using SelfEntersBattlefield
//! as approximation.
//! GAP: "put cards your opponents own from exile into graveyards" — no
//! catalog Effect for this. Token creation implemented (three times).

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
    let name = reg.interner_mut().intern("Blight Herder");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let processor = reg.interner_mut().intern("Processor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(processor);
    let _scion = reg.interner_mut().intern("Scion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
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
                // GAP: "when you cast this spell" — using ETB as approximation.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_scions,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_create_scions(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put two exile cards to graveyard" rider omitted.
    let scion = reg.interner().lookup("Scion").expect("Scion interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scion);
    let make_token = || {
        let mut st = SubtypeSet::default();
        st.0.insert(scion);
        TokenDefinition {
            name: scion,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes: st,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        }
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: make_token() },
        Effect::CreateToken { controller: trig.controller, token: make_token() },
        Effect::CreateToken { controller: trig.controller, token: make_token() },
    ]
}
