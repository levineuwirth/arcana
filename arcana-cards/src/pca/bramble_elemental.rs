//! Bramble Elemental — `{3}{G}{G}` 4/4 Creature — Elemental.
//! "Whenever an Aura becomes attached to this creature, create two 1/1 green Saproling creature tokens."
//! GAP: AuraBecomeAttached not a standard TriggerCondition; using SelfEntersBattlefield.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bramble Elemental");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let _saproling = reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: bramble_elemental_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn bramble_elemental_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let saproling_tok = reg.interner().lookup("Saproling").expect("Saproling interned during register()");
    let mut subtypes_0 = SubtypeSet::default();
    subtypes_0.0.insert(saproling_tok);
    let mut subtypes_1 = SubtypeSet::default();
    subtypes_1.0.insert(saproling_tok);
    vec![
        Effect::CreateToken { controller: trig.controller, token: TokenDefinition { name: saproling_tok, colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes: subtypes_0, power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)), keywords: vec![], abilities: vec![] } },
        Effect::CreateToken { controller: trig.controller, token: TokenDefinition { name: saproling_tok, colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes: subtypes_1, power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)), keywords: vec![], abilities: vec![] } },
    ]
}
