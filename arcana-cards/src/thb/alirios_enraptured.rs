//! Alirios, Enraptured — `{2}{U}` Legendary 2/3 Human.
//! Alirios enters tapped. (GAP: enters-tapped replacement not expressible.)
//! Alirios doesn't untap during your untap step if you control a Reflection.
//! (GAP: static untap restriction not expressible.)
//! When Alirios enters, create a 3/2 blue Reflection creature token.

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
    let name = reg.interner_mut().intern("Alirios, Enraptured");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    // Token subtype interned for the resolver.
    let _reflection = reg.interner_mut().intern("Reflection");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Alirios enters tapped" — enters-tapped replacement not expressible here.
    // GAP: "doesn't untap during your untap step if you control a Reflection" — static untap restriction not expressible.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_reflection,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_reflection(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let reflection = reg.interner().lookup("Reflection").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(reflection);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: reflection,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
